import { useState } from "react";
import axios from "axios";



const token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJOYXRpdmUiOnsic3ViIjozOSwic2Vzc2lvbl90eXBlIjoibmF0aXZlIn19.HG40BXb_BsUplDUEgZkpmLZNcM-o0D9YHTBJ6dKhNxM"; 

function App() {
    const [message, setMessage] = useState("");
    const [UUId, setUuid] = useState(null);
    const [socket, setSocket] = useState(null);
  
    const getUUID = async () => {
      try {
        /*
        const response = await axios.get("http://localhost:8000/ws/auth", {
          headers: {
            Authorization: token,
          }
        });
        const UUID = response.data.uuid;
        
        */
        // Connessione Socket.IO con UUID
        const socket= new WebSocket("ws://localhost:8000/ws/app?uuid=1b203946-a0a0-401a-ac40-0938b19db9b8",)

        socket.onopen = () => {
          console.log('Connected to WebSocket server');
        };
        
        socket.onmessage = (event) => {
          console.log(`Received message: ${event.data}`);
        };

  
        
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
