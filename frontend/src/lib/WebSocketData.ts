export function GetData(data:string){
    const dataJson= JSON.parse(data)
    const inputChannelStates=dataJson.i_mute
    const outputChannelStates=dataJson.o_mute
    
    return { inputChannelStates,outputChannelStates }   
}
