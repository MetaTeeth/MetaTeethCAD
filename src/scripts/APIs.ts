// Remote
import axios from "axios";

const _USER_NAME_ = "GET MY TOKEN";
const _USER_PASSWORD_ = "b19aa580-90e1-4f32-b065-e5f4c1b9c2cd";
const _API_HOST_ = "https://dental.scubot.com";

export function APIPost(
  endpoint: string,
  body: {},
  params: {},
  onSuccess: Function = () => {},
  onFailed: Function = () => {},
  contentType: string = "application/json",
) {
  axios
    .post(`${_API_HOST_}${endpoint}`, body, {
      headers: {
        'Content-Type': contentType,
      },
      auth: {
        username: _USER_NAME_,
        password: _USER_PASSWORD_
      },
      params: params
    })
    .then((resp) => { onSuccess(resp); })
    .catch((err) => { onFailed(err); });
}

export function APIRegister(bin: ArrayBuffer, onSuccess: Function, onFailed: Function) {
  const blob = new Blob([bin]);
  const formData = new FormData();
  formData.append('file', blob, '');

  APIPost('/register', formData, {}, onSuccess, onFailed, 'multipart/form-data');
}
