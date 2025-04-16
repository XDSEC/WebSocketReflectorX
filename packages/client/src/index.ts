import {
  WSRX_MINIMUM_REQUIRED,
  WsrxError,
  WsrxErrorKind,
  WsrxFeature,
  WsrxInstance,
  WsrxOptions,
  WsrxState,
} from "./types";
import ky, { HTTPError } from "ky";

/**
 * WebSocket Reflector X (wsrx) client.
 */
class Wsrx {
  private options: WsrxOptions;
  private state: WsrxState;
  private instances: WsrxInstance[];
  private onStateChangeCallbacks: ((state: WsrxState) => void)[];
  private onInstancesChangeCallbacks: ((instances: WsrxInstance[]) => void)[] =
    [];
  private interval: ReturnType<typeof setInterval> | null = null;

  private tickCounter = 0;

  /**
   * Creates a new Wsrx client.
   * @param options - The options for the Wsrx client.
   */
  constructor(options: WsrxOptions) {
    this.options = options;
    this.state = WsrxState.Invalid;
    this.instances = [];
    this.onStateChangeCallbacks = [];
  }

  /**
   * Sets the state of the Wsrx client and calls the state change callbacks.
   * @param state - The new state of the Wsrx client.
   *
   * NOTE: This method is private and should not be called directly.
   */
  private setState(state: WsrxState): void {
    if (this.state === state) {
      return;
    }
    this.state = state;
    for (const cb of this.onStateChangeCallbacks) {
      cb(state);
    }
  }

  /**
   * Syncs instances with local wsrx daemon.
   *
   * This method will be automatically called every 15 seconds when the client is in the usable state.
   *
   * You can also call this method manually to sync the instances immediately.
   * For example, if you want to sync the instances after adding or deleting an instance.
   *
   * @throws WsrxError if the sync fails.
   */
  public async sync() {
    try {
      const data: WsrxInstance[] = await ky
        .get(`${this.options.api}/pool`)
        .json();
      let diff = false;
      for (const i of data) {
        const index = this.instances.findIndex((j) => j.local === i.local);
        if (index === -1) {
          diff = true;
          break;
        }
      }
      for (const i of this.instances) {
        const index = data.findIndex((j) => j.local === i.local);
        if (index === -1) {
          diff = true;
          break;
        }
      }
      this.instances = data;
      if (diff) {
        for (const cb of this.onInstancesChangeCallbacks) {
          cb(data);
        }
      }
    } catch (e) {
      if (e instanceof HTTPError) {
        throw new WsrxError(WsrxErrorKind.DaemonError, await e.response.text());
      } else {
        throw new WsrxError(
          WsrxErrorKind.DaemonError,
          "Failed to sync instances, is wsrx daemon running?",
        );
      }
    }
  }

  /**
   * Starts the tick interval to check the state of the wsrx client.
   *
   * This method will be automatically called when the client is connected.
   * Because of that, you should not call this method directly.
   *
   * @returns A promise that resolves when the tick interval is started.
   */
  private async tick() {
    if (this.interval !== null) {
      clearInterval(this.interval);
    }
    this.interval = setInterval(async () => {
      await this.check().catch(() => {});
      if (this.state === WsrxState.Invalid) {
        this.interval && clearInterval(this.interval);
      } else if (this.state === WsrxState.Usable) {
        if (this.tickCounter % 5 === 0) {
          await this.sync().catch(() => {
            this.setState(WsrxState.Invalid);
          });
        }
        this.tickCounter++;
        this.tickCounter %= 5;
      }
    }, 3000);
  }

  /**
   * Returns the current state of the wsrx client.
   */
  public getState(): WsrxState {
    return this.state;
  }

  /**
   * Returns the current options of the wsrx client.
   */
  public getOptions(): WsrxOptions {
    return this.options;
  }

  /**
   * Sets the options of the Wsrx client.
   *
   * You should call `connect` after setting the options to apply the changes.
   *
   * @param options - The new options for the wsrx client.
   */
  public setOptions(options: Partial<WsrxOptions>): void {
    this.options = { ...this.options, ...options };
  }

  /**
   * Connects to the local wsrx daemon.
   *
   * This method will automatically check the state of the Wsrx client
   * and start the tick interval to check the state every second.
   *
   * @throws WsrxError if the connection fails.
   */
  public async connect(): Promise<void> {
    try {
      await ky.post(`${this.options.api}/connect`, {
        json: {
          name: this.options.name,
          features: this.options.features,
          host: "IN_HEADER",
          state: "pending",
        },
      });
    } catch {
      this.setState(WsrxState.Invalid);
      throw new WsrxError(
        WsrxErrorKind.DaemonUnavailable,
        "Failed to connect to wsrx daemon, is it running?",
      );
    }
    this.tick();
  }

  /**
   * Checks the version of the wsrx client.
   *
   * This method will check the version of the wsrx client and throw an error if the version is lower than required.
   *
   * @throws WsrxError if the version is lower than required.
   */
  public async checkVersion() {
    try {
      const data: { version: string } = await ky
        .get(`${this.options.api}/version`)
        .json();

      if (data.version < WSRX_MINIMUM_REQUIRED) {
        this.setState(WsrxState.Invalid);
        throw new WsrxError(
          WsrxErrorKind.VersionMismatch,
          `wsrx version ${data.version} is lower than required ${WSRX_MINIMUM_REQUIRED}`,
        );
      }
    } catch (e) {
      if (e instanceof HTTPError) {
        this.setState(WsrxState.Invalid);
        if (e.response.status === 404)
          throw new WsrxError(
            WsrxErrorKind.VersionMismatch,
            `wsrx version is lower than required ${WSRX_MINIMUM_REQUIRED}`,
          );
        else
          throw new WsrxError(
            WsrxErrorKind.DaemonError,
            "Failed to check wsrx version, is it running?",
          );
      }
    }
  }

  /**
   * Checks the state of the wsrx client.
   *
   * This method will check the state of the wsrx client and return the state.
   * It will also start the tick interval to check the state every second.
   */
  public async check(): Promise<void> {
    try {
      const resp = await ky.get(`${this.options.api}/connect`);
      if (resp.status === 202) {
        this.setState(WsrxState.Usable);
      } else if (resp.status === 201) {
        this.setState(WsrxState.Pending);
        throw new WsrxError(
          WsrxErrorKind.ScopeUnverified,
          "Your scope is pending, please verify this website in local daemon",
        );
      } else {
        this.setState(WsrxState.Invalid);
        throw new WsrxError(
          WsrxErrorKind.DaemonError,
          "Daemon returns unexpected status code, is it ok?",
        );
      }
    } catch (e) {
      if (e instanceof HTTPError) {
        if (e.response.status === 404) {
          this.setState(WsrxState.Invalid);
          throw new WsrxError(
            WsrxErrorKind.DaemonUnavailable,
            "Failed to connect to wsrx daemon, is it running?",
          );
        } else if (e.response.status === 403) {
          this.setState(WsrxState.Invalid);
          throw new WsrxError(
            WsrxErrorKind.MissingScope,
            "Your scope is missing in local daemon, please call `connect` first",
          );
        }
      } else {
        this.setState(WsrxState.Invalid);
        throw new WsrxError(
          WsrxErrorKind.DaemonUnavailable,
          "Failed to connect to wsrx daemon, is it running?",
        );
      }
    }
  }

  /**
   * Adds a new instance to the wsrx client.
   *
   * It will also notify callbacks.
   *
   * @param instance - The instance to add.
   * @returns The added instance.
   */
  public async add(instance: WsrxInstance): Promise<WsrxInstance> {
    try {
      for (const i of this.instances) {
        if (i.remote === instance.remote) {
          return i;
        }
      }
      const data: WsrxInstance = await ky
        .post(`${this.options.api}/pool`, {
          json: instance,
        })
        .json();
      this.instances.push(data);
      for (const cb of this.onInstancesChangeCallbacks) {
        cb(this.instances);
      }
      return data;
    } catch (e) {
      if (e instanceof HTTPError) {
        throw new WsrxError(WsrxErrorKind.DaemonError, await e.response.text());
      }
    }
    throw new WsrxError(
      WsrxErrorKind.DaemonError,
      "Failed to add instance, is wsrx daemon running?",
    );
  }

  /**
   * Returns the instance with the given local address.
   *
   * @param local - The local address of the instance.
   * @returns The instance with the given local address, or null if not found.
   */
  public get(local: string): WsrxInstance | null {
    return this.instances.find((i) => i.local === local) || null;
  }

  /**
   * Deletes the instance with the given local address.
   *
   * @param local - The local address of the instance to delete.
   */
  public async delete(local: string): Promise<void> {
    try {
      await ky.delete(`${this.options.api}/pool`, {
        json: { local },
      });
      this.instances = this.instances.filter((i) => i.local !== local);
      for (const cb of this.onInstancesChangeCallbacks) {
        cb(this.instances);
      }
    } catch (e) {
      if (e instanceof HTTPError) {
        throw new WsrxError(WsrxErrorKind.DaemonError, await e.response.text());
      }
      throw new WsrxError(
        WsrxErrorKind.DaemonError,
        "Failed to delete instance, is wsrx daemon running?",
      );
    }
  }

  /**
   * Returns the list of instances.
   *
   * @returns The list of instances.
   */
  public list(): WsrxInstance[] {
    return this.instances;
  }

  /**
   * Returns the list of features.
   *
   * @returns The list of features.
   */
  public onStateChange(fn: (state: WsrxState) => void): void {
    this.onStateChangeCallbacks.push(fn);
  }

  /**
   * Returns the list of features.
   *
   * @returns The list of features.
   */
  public onInstancesChange(fn: (instances: WsrxInstance[]) => void): void {
    this.onInstancesChangeCallbacks.push(fn);
  }
}

export {
  Wsrx,
  WsrxOptions,
  WsrxInstance,
  WsrxState,
  WsrxFeature,
  WsrxErrorKind,
  WsrxError,
};
