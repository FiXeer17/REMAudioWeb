import Navbar from "@/components/ui/navbar"
import InOutButton from "@/components/ui/in_out"
import { Button as Preset } from "@/components/ui/button_mute";

export default function Volume(){
    return(
        <div className="grid grid-rows-[1/3fr,auto,1fr] mx-5 min-h-svh">
            <div className="flex items-center justify-center">
                <Preset variant={"preset"} size={"preset"}>
                    PRESET
                </Preset>
            </div>
            <div className="flex ">
                <div>

                </div>
                <div className="flex gap-2">
                    <div>
                        <InOutButton/>
                    </div>
                    <div>
                        <InOutButton/>
                    </div>
                    <div>
                        <InOutButton/>
                    </div>
                    <div>
                        <InOutButton/>
                    </div>
                </div>

            </div>
            <div className="flex flex-col justify-end items-center pb-3">
                <Navbar selectedColor="speaker"/>
            </div>
        </div>
    )
}