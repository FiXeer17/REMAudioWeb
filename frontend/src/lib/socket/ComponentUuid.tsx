import { createContext, useContext, useEffect,useState } from "react";
import { getUUID, getSocket } from "../services";
import { useLocation } from "react-router-dom";


interface ConnectionsContextType {
  uuid: string | undefined;
  sockets: { name: string; ip: string; port: number; device_type:string; isLatestAudio?: boolean; isLatestVideo?: boolean;}[] | null;
  isAdmin:boolean;
  triggerRedirect: () => void;
}
const ConnectionsContext = createContext<ConnectionsContextType | undefined>(undefined);


export const ConnectionsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [uuid, setUuid] = useState<string>();
  const [prevUuid, setPrevUuid] = useState<string>();
  const [sockets, setSockets] = useState<{ name: string; ip: string; port: number; device_type:string }[] | null>(null);
  const isAdmin = localStorage.getItem("isAdmin") === "true";

  

  useEffect(() => {
    const fetchUUID = async () => {
      try {
        const value = await getUUID();
        setPrevUuid(value.data.uuid);
      } catch (error) {
        console.error("Error fetching UUID:", error);
      }
    };
    fetchUUID();
  }, []);

  const fetchSocket = async () => {
    if (!prevUuid) return;

    try {
      const value = await getSocket();
      const latest_audio = value.data.latest_audio_socket;
      const latest_video = value.data.latest_video_socket;
      if (isAdmin) {
        
        const allSockets = value.data.sockets ;
        const updatedSockets = allSockets || latest_audio || latest_video
        ? [
          ...(allSockets ?? []),
          ...(latest_audio ? [{ ...latest_audio, isLatestAudio: true }] : []),
          ...(latest_video ? [{ ...latest_video, isLatestVideo: true }] : []),
        ]:null
        
        setSockets(updatedSockets)

      } else {
        if (latest_audio || latest_video) {
          const updatedSockets = [
            ...(latest_audio ? [{ ...latest_audio, isLatestAudio: true }] : []),
            ...(latest_video ? [{ ...latest_video, isLatestVideo: true }] : []),
          ];
          setSockets(updatedSockets);
        } else {
          setSockets(null);
        }
      }
      setUuid(prevUuid)
    } catch (error) {
      console.error("Error getting Socket:", error);
    }  
  }

  useEffect(() => {
    if (prevUuid) {
      fetchSocket();
    }
  }, [prevUuid]);

 

  return (

    <ConnectionsContext.Provider value={{ uuid, sockets,isAdmin, triggerRedirect: fetchSocket }}>
        
        {children}
    </ConnectionsContext.Provider>

  );
};
export default ConnectionsProvider;

export const useConnections = (): ConnectionsContextType => {
  const context = useContext(ConnectionsContext);
  if (!context) {
    throw new Error("useConnections must be used within a UUIDProvider");
  }
  return context;
};

