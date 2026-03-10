/**
 * パスワードの強度をチェック
 */
const PASSWORD_PATTERN = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[~`!@#$%^&*()_+-={}[|;:'",<.>/?])(?=.{8,})/;

export function validatePassword(password: string) {
  return PASSWORD_PATTERN.test(password);
}