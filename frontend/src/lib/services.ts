import { client } from "./axiosClient";
import { AxiosResponse } from 'axios';

interface UserCredentials {
    username:string;
    email?: string;
    password: string;
    session_type: string;

  }
interface UserSocket{
  uuid:string|undefined;
  socket:string;
  socket_name:string;

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

export function getSocket():Promise<AxiosResponse<{ sockets: { name: string; ip: string; port: number }[] | null;latest_socket: { name: string; ip: string; port: number } | null;}>>{
  return client.get("api")
}

export function setSocket({uuid,socket_name,socket}: UserSocket): Promise<AxiosResponse> {
  return client.post(`ws/socket/add?uuid=${uuid}`, {socket_name,socket});
}