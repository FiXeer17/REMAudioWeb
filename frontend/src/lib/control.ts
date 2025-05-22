import { redirect } from 'react-router-dom';

export async function clientLoader() {
  try {
    const accessToken = localStorage.getItem("accessToken");
        
    if (!accessToken) {
      return redirect("/login");
    }

  } catch (error) {
    localStorage.removeItem("accessToken");

    return redirect("/login");
  }
}

export async function isAdmin() {
  try{
    const isAdmin=localStorage.getItem("isAdmin")
    if(isAdmin==="false"){
      return redirect("/")
    }
  }catch(error){
    return 
  }
}