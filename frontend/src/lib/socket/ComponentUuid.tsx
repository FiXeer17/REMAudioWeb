import { createContext, useContext, useEffect, useState } from "react";
import { getUUID, getSocket } from "../services";
import { useNavigate, useLocation } from "react-router-dom";
import { setSocket } from "../services";

interface UUIDContextType {
  uuid: string | undefined;
  sockets: { name: string; ip: string; port: number }[] | null;
}
const UUIDContext = createContext<UUIDContextType | undefined>(undefined);
const SocketContext = createContext<UUIDContextType | undefined>(undefined);

export const UUIDProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const navigate = useNavigate();
  const [uuid, setUuid] = useState<string>();
  const [sockets, setSockets] = useState<{ name: string; ip: string; port: number }[] | null>(null);
  const location = useLocation();
  const { isAdmin } = location.state || {};

  useEffect(() => {
    const fetchUUID = async () => {
      try {
        const value = await getUUID();
        setUuid(value.data.uuid);
      } catch (error) {
        console.error("Error fetching UUID:", error);
      }
    };
    fetchUUID();
  }, []);
  useEffect(() => {
    if (!uuid) return;
    const fetchSocket = async () => {
      try {
        const value = await getSocket();
        if (isAdmin) {
          setSockets(value.data.sockets);
          if (value.data.sockets === null) 
            return navigate("/newconnections");
          else 
            return navigate("/recentconnections");
        } else {
          
          setSockets(
            value.data.latest_socket ? [value.data.latest_socket] : null
          );
          
          
              if (value.data.latest_socket !== null) {
                const headers = {
                  uuid: uuid,
                  socket_name: value.data.latest_socket.name,
                  socket: `${value.data.latest_socket.ip}:${value.data.latest_socket.port}`,
                };
                console.log(headers)
                const response = await setSocket(headers);
                if (response.status === 200) {
                  return navigate("/homeAudio");
                }
                
              } else {
                console.log("no socket")
                return navigate("/login")
              }
              
        }
      } catch (error) {
        console.error("Error getting Socket:", error);
      }
    };
    fetchSocket();
  }, [uuid]);

  return (
    <UUIDContext.Provider value={{ uuid, sockets }}>
      <SocketContext.Provider value={{ uuid, sockets }}>
        {children}
      </SocketContext.Provider>
    </UUIDContext.Provider>
  );
};
export default UUIDProvider;

export const useUUID = (): UUIDContextType => {
  const context = useContext(UUIDContext);
  if (!context) {
    throw new Error("useUUID must be used within a UUIDProvider");
  }
  return context;
};
export const useSockets = (): UUIDContextType => {
  const context = useContext(SocketContext);
  if (!context) {
    throw new Error("useUUID must be used within a UUIDProvider");
  }
  return context;
};
