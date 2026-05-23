export const decodeJwtPayload = (token: string): any => {
  const payload = token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/');
  const paddedPayload = payload.padEnd(Math.ceil(payload.length / 4) * 4, '=');
  return JSON.parse(atob(paddedPayload));
};

export const decodeAccessToken = decodeJwtPayload;
