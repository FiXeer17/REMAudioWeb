import { useLocation, useNavigate } from "react-router-dom";
import { useEffect} from "react";
import { useConnections } from "@/lib/socket/ComponentUuid"; 

const UUIDLayout = () => {
  const {uuid,sockets, isAdmin } = useConnections(); 
  const navigate =useNavigate()
  const location=useLocation()


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
          if(sockets.some(socket => socket.isLatestVideo) && !sockets.some(socket => socket.isLatestAudio))
            navigate("/video");
          else
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
    <></>
  );
};

export default UUIDLayout;
