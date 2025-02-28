import { client } from "./axiosClient";
import { AxiosResponse } from 'axios';

interface UserCredentials {
    username?:string;
    email: string;
    password: string;
    session_type: string;

  }
  
interface UserProfile {
    id: number;
    email: string;
    name: string;
  }

// Funzione per la registrazione
export function register({ username, email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("/register", { username, email, password, session_type });
}

// Funzione per il login
export function login({ email, password, session_type }: UserCredentials): Promise<AxiosResponse> {
  return client.post("/signin", { email, password, session_type });
}

// Funzione per ottenere il profilo dell'utente
export function getProfile(): Promise<AxiosResponse<UserProfile>> {
  return client.get("/profile");
}
