import { useNavigate } from "react-router-dom";
import { useEffect} from "react";
import { useConnections } from "@/lib/socket/ComponentUuid"; // Assicurati di usare il contesto giusto

const UUIDLayout = () => {
  const {uuid,sockets, isAdmin } = useConnections(); // Usa il contesto per ottenere i dati
  const navigate =useNavigate()

  
  useEffect(() => {
    const handleRedirect = async () => {
      
      if(!sockets&&!uuid) return
      if (isAdmin) {
        if (sockets === null) {
          navigate("/newconnections");
        } else {
          navigate("/recentconnections");
        }
      } else {
        if (sockets !== null) {
          navigate("/homeAudio");
        } else {
          navigate("/callAdministrator");
        }
      }
    };

    handleRedirect();
  }, [uuid,sockets, isAdmin]);

  return (
    <div>
      <p>Caricamento in corso...</p>
    </div>
  );
};

export default UUIDLayout;
