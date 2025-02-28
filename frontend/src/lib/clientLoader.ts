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