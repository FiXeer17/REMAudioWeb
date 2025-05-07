import NavbarDesktop from "@/components/ui/navbarDesktop"
import { BookBookmark,Translate,Network,SignOut } from "@phosphor-icons/react"
import { Avatar,AvatarImage } from "@/components/ui/avatar"
import { useNavigate } from "react-router-dom"
import { useConnections } from "@/lib/socket/ComponentUuid"

export const Settings=()=>{
    const navigate=useNavigate()
    const { isAdmin,triggerRedirect } = useConnections();

    const handleRedirect = async () => {
        await triggerRedirect()
        navigate("/uuidprovider",{state:{show:false}})
        }
    const signOut=()=>{
        localStorage.removeItem("accessToken")
        navigate("/login")
    }
    return(
        <div className="grid grid-cols-[100px,1fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="settings" />
            </div>
            <div className="flex items-center justify-center pr-10">
                <div className="grid grid-rows-[1fr,1fr,1/2fr] border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[650px] w-[600px]  px-24">
                    <div className="flex flex-col items-center justify-center pt-5 gap-4">
                        <Avatar className="h-32 w-32">
                            <AvatarImage className="" src="./Avatars.svg"/>
                        </Avatar>   
                        <p className="text-home_colors-Similar_White font-bold text-sm">@{localStorage.getItem("user")}</p>
                    </div>
                    <div className="flex flex-col justify-between h-full ">
                        <div className="flex-1">
                            <div className="flex gap-1 h-full items-center pl-10 w-fit cursor-pointer" onClick={()=>navigate("/preferenciesPresets")}>
                                <BookBookmark color="#FAFAFA" size={30} weight="light" />
                                <p className="flex font-bold text-sm  text-home_colors-Similar_White">Change preferencies</p>
                            </div>
                        </div>
                        <div className="flex-1 border-y-[0.7px] border-home_colors-Border_Connections border-opacity-45 pl-10">
                            <div className="flex gap-1 h-full items-center w-fit cursor-pointer">
                                <Translate color="#FAFAFA" size={30} weight="light" />
                                <p className="flex font-bold text-sm text-home_colors-Similar_White">Change language</p>
                            </div>
                        </div>
                        {isAdmin &&(
                        <div className="flex-1 border-b-[0.7px] border-home_colors-Border_Connections border-opacity-45 pl-10">
                            <div className="flex gap-1 h-full items-center w-fit cursor-pointer" onClick={()=>handleRedirect()}>
                                <Network color="#FAFAFA" size={30} weight="light" />
                                <p className="flex font-bold text-sm text-home_colors-Similar_White">Change connections</p>
                            </div>
                        </div>
                        )}
                        <div className="flex-1 pl-10 pt-5">
                            <div className="flex gap-1 h-full items-center w-fit cursor-pointer" onClick={()=>signOut()}>
                                <SignOut color="#F37171" size={30} weight="light" />
                                <p className="flex font-bold text-sm text-[#F37171]">Sign out</p>
                            </div>
                        </div>
                    </div>
                    <div/>
                </div>
            </div>
        </div>
    )
}