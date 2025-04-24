import { useLocation, useNavigate } from "react-router-dom";
import { useEffect} from "react";
import { useConnections } from "@/lib/socket/ComponentUuid"; // Assicurati di usare il contesto giusto

const UUIDLayout = () => {
  const {uuid,sockets, isAdmin } = useConnections(); // Usa il contesto per ottenere i dati
  const navigate =useNavigate()
  const location=useLocation()

  useEffect(()=>{
    console.log(location.state?.show)
  },[sockets])

  useEffect(() => {
    const handleRedirect = async () => {
      
      if(!sockets&&!uuid) return
      if (isAdmin) {
        if (sockets === null) {
          if (location.state?.show === true) {
            navigate("/newconnections", { state: { show: true } });
          } else {
            navigate("/newconnections",{ state: { show: false } });
          }
        } else {
          if (location.state?.show === true) {
            navigate("/recentConnections", { state: { show: true } });
          } else {
            navigate("/recentConnections", { state: { show: false } });
          }
        }
      } else {
        if (sockets !== null) {
          navigate("/homeAudio");
        } else {
          if (location.state?.show === true) {
            navigate("/callAdministrator", { state: { show: true } });
          } else {
            navigate("/callAdministrator");
          }
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
