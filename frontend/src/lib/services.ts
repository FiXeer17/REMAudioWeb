import { client } from "./axiosClient";
import { AxiosResponse } from 'axios';

interface UserCredentials {
    username?:string;
    email: string;
    password: string;
    session_type: string;

  }
  

export function register({ username, email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("/register", { username, email, password, session_type });
}


export function login({ email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("/signin", { email, password, session_type });
}


