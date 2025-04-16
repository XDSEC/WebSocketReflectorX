import {
	WSRX_MINIMUM_REQUIRED,
	WsrxFeature,
	WsrxInstance,
	WsrxOptions,
	WsrxState,
} from "./types";

/**
 * WebSocket Reflector X (wsrx) client.
 */
class Wsrx {
	private options: WsrxOptions;
	private state: WsrxState;
	private instances: WsrxInstance[];
	private onStateChangeCallbacks: ((state: WsrxState) => void)[];
	private onInstancesChangeCallbacks: (() => void)[] = [];
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
	 * @returns A promise that resolves when the sync is complete.
	 */
	public async sync() {
		try {
			const resp = await fetch(`${this.options.api}/pool`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			});
			if (resp.ok) {
				const data: WsrxInstance[] = await resp.json();
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
						cb();
					}
				}
			}
		} catch (e) {}
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
			const state = await this.check().catch(() => {});
			if (state) this.setState(state);
			else this.setState(WsrxState.Invalid);
			if (this.state === WsrxState.Invalid) {
				this.interval && clearInterval(this.interval);
			} else if (this.state === WsrxState.Usable) {
				if (this.tickCounter % 15 === 0) {
					await this.sync();
				}
				this.tickCounter++;
				this.tickCounter %= 15;
			}
		}, 1000);
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
	 * @param onError - Optional callback to handle errors.
	 */
	public async connect(onError?: (e: Error) => void): Promise<void> {
		try {
			const resp = await fetch(`${this.options.api}/connect`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					name: this.options.name,
					features: this.options.features,
					host: "IN_HEADER",
					state: "pending",
				}),
			});
			if (!resp.ok) {
				this.setState(WsrxState.Invalid);
			}
			this.tick();
		} catch (e) {
			if (onError) {
				onError(e as Error);
			} else {
				throw e;
			}
		}
	}

	public async checkVersion() {
		const version = await fetch(`${this.options.api}/version`, {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
			},
		});
		if (!version.ok) {
			this.setState(WsrxState.Invalid);
			return WsrxState.Invalid;
		}
		if (version.status === 404) {
			this.setState(WsrxState.Invalid);
			onError?.(
				new Error(
					`The minimum required version of wsrx is ${WSRX_MINIMUM_REQUIRED}, please update your wsrx client.`,
				),
			);
			return WsrxState.Invalid;
		}
		const data = await version.json();
		if (data.version < WSRX_MINIMUM_REQUIRED) {
			this.setState(WsrxState.Invalid);
			onError?.(
				new Error(
					`The minimum required version of wsrx is ${WSRX_MINIMUM_REQUIRED}, please update your wsrx client.`,
				),
			);
			return WsrxState.Invalid;
		}
	}

	/**
	 * Checks the state of the wsrx client.
	 *
	 * This method will check the state of the wsrx client and return the state.
	 * It will also start the tick interval to check the state every second.
	 *
	 * @param onError - Optional callback to handle errors.
	 * @returns The state of the wsrx client.
	 */
	public async check(onError?: (e: Error) => void): Promise<WsrxState> {
		try {
			const resp = await fetch(`${this.options.api}/connect`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			});
			if (resp.ok) {
				if (resp.status === 202) {
					this.setState(WsrxState.Usable);
					return WsrxState.Usable;
				} else if (resp.status === 201) {
					this.setState(WsrxState.Pending);
					return WsrxState.Pending;
				} else {
					this.setState(WsrxState.Invalid);
					return WsrxState.Invalid;
				}
			} else {
				this.setState(WsrxState.Invalid);
				return WsrxState.Invalid;
			}
		} catch (e) {
			if (onError) {
				onError(e as Error);
			} else {
				throw e;
			}
		}
		return this.state;
	}

	/**
	 * Adds a new instance to the wsrx client.
	 *
	 * @param instance - The instance to add.
	 * @param onError - Optional callback to handle errors.
	 * @returns The added instance.
	 */
	public async add(
		instance: WsrxInstance,
		onError?: (e: Error) => void,
	): Promise<WsrxInstance> {
		try {
			for (const i of this.instances) {
				if (i.remote === instance.remote) {
					return i;
				}
			}

			const resp = await fetch(`${this.options.api}/pool`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(instance),
			});
			if (resp.ok) {
				const data = await resp.json();
				this.instances.push(data);
				for (const cb of this.onInstancesChangeCallbacks) {
					cb();
				}
				return data;
			} else {
				throw new Error("Failed to add instance");
			}
		} catch (e) {
			if (onError) {
				onError(e as Error);
			} else {
				throw e;
			}
		}
		throw new Error("Failed to add instance");
	}

	public get(local: string): WsrxInstance | null {
		return this.instances.find((i) => i.local === local) || null;
	}

	public async delete(
		local: string,
		onError?: (e: Error) => void,
	): Promise<void> {
		try {
			const resp = await fetch(`${this.options.api}/pool`, {
				method: "DELETE",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ local }),
			});
			if (resp.ok) {
				this.instances = this.instances.filter((i) => i.local !== local);
				for (const cb of this.onInstancesChangeCallbacks) {
					cb();
				}
			} else {
				throw new Error("Failed to delete instance");
			}
		} catch (e) {
			if (onError) {
				onError(e as Error);
			} else {
				throw e;
			}
		}
	}

	public list(): WsrxInstance[] {
		return this.instances;
	}

	public onStateChange(fn: (state: WsrxState) => void): void {
		this.onStateChangeCallbacks.push(fn);
	}

	public onInstancesChange(fn: () => void): void {
		this.onInstancesChangeCallbacks.push(fn);
	}
}

export { Wsrx, WsrxOptions, WsrxInstance, WsrxState, WsrxFeature };
