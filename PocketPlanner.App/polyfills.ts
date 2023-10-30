import { decode, encode } from "base-64"

(function base64() {
  global.btoa = encode
  global.atob = decode
})()
