const _api = process.env.NEXT_PUBLIC_API || "http://localhost:8080";
const _url = new URL(_api);
const _ws_protocol = _url.protocol === "https:" ? "wss:" : "ws:";
export const WS_URL = `${_ws_protocol}//${_url.host}${_url.pathname.replace(/\/$/g, "")}/ws`;
