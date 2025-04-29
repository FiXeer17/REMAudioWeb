import Navbar from "@/components/ui/navbar"
import InOutButton from "@/components/ui/in_out"
import { Button as Preset } from "@/components/ui/button_mute";
import { Button as Mute } from "@/components/ui/button_mute";
import { Slider } from "@/components/ui/slider";
import { Circle } from "@phosphor-icons/react";
import { SwipeVolumes } from "@/lib/swipeSliders";

export const Volume=()=>{
    
	const i_volumes= {
		"1": -2,
		"2": -130,
		"3": -130,
		"4": -2,
        "5": -2,
		"6": -130,
		"7": -120,
		"8": -2,
	}
    
    const {
        displayedVolumes:displayedVolumes,
        offset:Offset,
        handleTouchStart: handleInputTouchStart,
        handleTouchMove: handleInputTouchMove,
        handleTouchEnd: handleInputTouchEnd,
      } = SwipeVolumes(i_volumes);
    return(
        <div className="grid grid-rows-[0.5fr_2fr,auto] mx-5 min-h-svh">
            <div className="flex items-center justify-center">
                <Preset variant={"preset"} size={"preset"}>
                    PRESET
                </Preset>
            </div>
            <div className="flex justify-center">
                <div className="flex gap-2 pb-3  "
                    style={{
                        transform: `translateX(${Offset}px)`,
                        transition: Offset === 0 ? "transform 0.3s ease" : "none",
                    }}
                    onTouchStart={handleInputTouchStart}
                    onTouchMove={handleInputTouchMove}
                    onTouchEnd={handleInputTouchEnd}
                >
                    {Object.entries(displayedVolumes).map(([key, value]) => (
                        <div className="flex flex-col items-center gap-3" key={key} >
                            <p className="text-home_colors-Similar_White text-sm font-bold">{value} db</p>
                            <Slider orientation="vertical" className="h-full "/>
                            <p className="text-home_colors-Similar_White text-sm font-bold">CH{key}</p>
                            <InOutButton/>
                            <Mute size={"mute_preset"}>
                                MUTE
                            </Mute>
                        </div>
                    ))}
                
                </div>
            </div>
            <div className="flex flex-col justify-end items-center gap-4 pb-3">
                <Circle size={12} color="#ffffff" />
                <Navbar selectedColor="speaker"/>
            </div>
        </div>
    )
}