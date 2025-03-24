import { useState } from "react";
import axios from "axios";
import { io } from "socket.io-client";

const token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJOYXRpdmUiOnsic3ViIjozOSwic2Vzc2lvbl90eXBlIjoibmF0aXZlIn19.HG40BXb_BsUplDUEgZkpmLZNcM-o0D9YHTBJ6dKhNxM"; 

function App() {
    const [message, setMessage] = useState("");
    const [UUId, setUuid] = useState(null);
    const [socket, setSocket] = useState(null);
  
    const getUUID = async () => {
      try {
        const response = await axios.get("https://3f76-151-42-175-197.ngrok-free.app/ws/auth", {
          headers: {
            Authorization: token,
            'ngrok-skip-browser-warning':'true'
          }
        });
        const UUID = response.data.uuid;
        
        
        // Connessione Socket.IO con UUID
        const newSocket = io("wss://3f76-151-42-175-197.ngrok-free.app/ws/app", {
          query: { uuid: UUID },
          extraHeaders:{'ngrok-skip-browser-warning':'true'} ,
          path: "/ws/app"
          
        });
  
        // Gestisci la connessione
        newSocket.on("connect", () => {
          console.log("Connected to Socket.IO server");
        });
  
        // Gestisci i messaggi in arrivo
        newSocket.on("message", (data) => {
          setMessage(data);
        });
  
        
      } catch (error) {
        setMessage("Errore nel recupero dell'UUID");
      }
    };
  
  ;

  return (
    <div className="flex flex-col">
      <button onClick={getUUID} className="text-white">Ottieni UUID</button>
      <h1 className="text-white">{message}</h1>
    </div>
  );
}

export default App;
