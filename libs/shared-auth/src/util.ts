import nextBase64 from "next-base64";

export const decodeAccessToken = (access_token: string): any => {
  const access_token_payload_base64 = access_token.split(".")[1];
  const access_token_payload = JSON.parse(nextBase64.decode(access_token_payload_base64
    .replace(/-/g, "+")
    .replace(/_/g, "/")));
  return access_token_payload;
}