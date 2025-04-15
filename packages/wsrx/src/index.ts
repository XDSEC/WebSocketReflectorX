import {
	WSRX_MINIMUM_REQUIRED,
	WsrxFeature,
	WsrxInstance,
	WsrxOptions,
	WsrxState,
} from "./types";

class Wsrx {
	private options: WsrxOptions;
	private state: WsrxState;
	private instances: WsrxInstance[];
	private onStateChangeCallbacks: ((state: WsrxState) => void)[];
	private onInstancesChangeCallbacks: (() => void)[] = [];
	private interval: ReturnType<typeof setInterval> | null = null;

	private tickCounter = 0;

	constructor(options: WsrxOptions) {
		this.options = options;
		this.state = WsrxState.Invalid;
		this.instances = [];
		this.onStateChangeCallbacks = [];
	}

	private setState(state: WsrxState): void {
		if (this.state === state) {
			return;
		}
		this.state = state;
		for (const cb of this.onStateChangeCallbacks) {
			cb(state);
		}
	}

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

	public getState(): WsrxState {
		return this.state;
	}

	public getOptions(): WsrxOptions {
		return this.options;
	}

	public setOptions(options: Partial<WsrxOptions>): void {
		this.options = { ...this.options, ...options };
	}

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

	public async check(onError?: (e: Error) => void): Promise<WsrxState> {
		try {
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

			const resp = await fetch(`${this.options.api}/connect`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			});
			if (resp.ok) {
				if (resp.status === 202) {
					this.setState(WsrxState.Usable);
					this.tick();
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
