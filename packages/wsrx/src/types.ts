export enum WsrxState {
	Invalid,
	Pending,
	Usable,
}

export enum WsrxFeature {
	Basic = "basic",
	Pingfall = "pingfall",
}

export interface WsrxOptions {
	/**
	 * Website name for local scope
	 */
	name: string;
	/**
	 * API base URL from local wsrx, typically `http://localhost:3307`
	 */
	api: string;
	/**
	 * Enabled features
	 */
	features: WsrxFeature[];
}

export interface WsrxInstance {
	/**
	 * Instance name, will be displayed in local wsrx UI
	 */
	label?: string;
	/**
	 * Remote WebSocket URL
	 */
	remote: string;
	/**
	 * Local proxy server addr, use `127.0.0.1:0` to bind to a random port
	 *
	 * The corresponding port will be returned when the instance is added
	 */
	local: string;
	/**
	 * Instance latency, in milliseconds, -1 means unusable
	 *
	 * This field is maintained by wsrx
	 */
	latency?: number;
}

export const WSRX_MINIMUM_REQUIRED = "0.4";
