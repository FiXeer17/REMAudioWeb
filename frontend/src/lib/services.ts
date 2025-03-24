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

export function getUUID():Promise<AxiosResponse<string>>{
  return client.get("",{baseURL: "https://3299-37-103-104-79.ngrok-free.app/ws/auth"})
}


