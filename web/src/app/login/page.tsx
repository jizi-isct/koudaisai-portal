'use client';
import {useState} from 'react';
import {SubmitHandler, useForm} from "react-hook-form";
import {paths} from "@/lib/auth_v1";
import createFetchClient from "openapi-fetch";

type Inputs = {
  m_address: string,
  password: string
}
const fetchClient = createFetchClient<paths>({
  baseUrl: process.env.NEXT_PUBLIC_AUTH_BASE_URL,
});

export default function Login() {
  const [error, setError] = useState<string>();
  const {
    register,
    handleSubmit,
    watch,
    formState: {errors},
  } = useForm<Inputs>()
  const onSubmit: SubmitHandler<Inputs> = async (inputs) => {
    const {data, response} = await fetchClient.POST(
      "/login",
      {
        body: {
          m_address: inputs.m_address,
          password: inputs.password
        }
      })

    if (data) {
      localStorage.setItem("exhibitor_refresh_token", data.refresh_token)
      localStorage.setItem("exhibitor_access_token", data.access_token)
      window.location.assign("/")
    } else {
      switch (response.status) {
        case 401:
          setError("mアドレスまたはパスワードが間違えています。")
          break;
        default:
          setError("内部エラー。開発者に問い合わせてください。")
          break;
      }
    }
  };

  return (
    <div>
      <h1>ログイン</h1>
      {error && <p style={{color: 'red'}}>{error}</p>}
      <form onSubmit={handleSubmit(onSubmit)}>
        <div>
          <label htmlFor="username">mアドレス:</label>
          <input
            {...register("m_address")}
            type="email"
          />
        </div>
        <div>
          <label htmlFor="password">パスワード:</label>
          <input
            {...register("password")}
            type="password"
          />
        </div>
        <button type="submit">ログイン</button>
      </form>
    </div>
  );
}