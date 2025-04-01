import { client } from "./axiosClient";
import { AxiosResponse } from 'axios';

interface UserCredentials {
    username:string;
    email?: string;
    password: string;
    session_type: string;

  }
  

export function register({ username, email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("/register", { username, email, password, session_type });
}


export function login({ username, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("api/auth/signin", { username, password, session_type });
}

export function getUUID():Promise<AxiosResponse<{uuid:string}>>{
  return client.get("ws/auth")
}


