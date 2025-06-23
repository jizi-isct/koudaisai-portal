'use client';
import {useState} from 'react';
import {SubmitHandler, useForm} from "react-hook-form";
import {login} from "@/lib";

type Inputs = {
    m_address: string,
    password: string
}

export default function Login() {
    const [error, setError] = useState<string>();
    const {
        register,
        handleSubmit
    } = useForm<Inputs>()
    const onSubmit: SubmitHandler<Inputs> = async (inputs) => {
        try {
            await login(inputs.m_address, inputs.password)
        } catch (e) {
            setError(`${e}`)
            return
        }
        window.location.assign("/")
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