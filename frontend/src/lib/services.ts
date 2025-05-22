import { createClient } from "./axiosClient";
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
  device_type:string;

}
interface UserRemoveSocket{
  uuid:string|undefined;
  socket:string;

}
  

export function register({ username, email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  let client = createClient();
  return client.post("/register", { username, email, password, session_type });
}


export function login({ username, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  let client = createClient();
  return client.post("api/auth/signin", { username, password, session_type });
}

export function getUUID():Promise<AxiosResponse<{uuid:string}>>{
  let client = createClient();
  return client.get("ws/auth")
}

export function getSocket():Promise<AxiosResponse<{ sockets: { name: string; ip: string; port: number; device_type:string }[] | null;latest_audio_socket: { name: string; ip: string; port: number; device_type:string } | null;latest_video_socket: { name: string; ip: string; port: number; device_type:string } | null;}>>{
  let client = createClient();
  return client.get("api")
}

export function setSocket({uuid,socket_name,socket,device_type}: UserSocket): Promise<AxiosResponse> {
  let client = createClient();
  return client.post(`ws/socket/add?uuid=${uuid}`, {socket_name,socket,device_type});
}

export function addSocket({uuid,socket_name,socket,device_type}: UserSocket): Promise<AxiosResponse> {
  let client = createClient();
  return client.post(`ws/socket/add?uuid=${uuid}`, {socket_name,socket,device_type});
}

export function removeSocket({uuid,socket}: UserRemoveSocket): Promise<AxiosResponse> {
  let client = createClient();
  return client.post(`ws/socket/remove?uuid=${uuid}`, {socket});
}