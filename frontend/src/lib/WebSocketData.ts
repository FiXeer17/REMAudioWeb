export function GetData(data:string){
    const dataJson= JSON.parse(data)
    const inputChannelStates=dataJson.i_mute
    const outputChannelStates=dataJson.o_mute
    const outputVisibility=dataJson.o_visibility
    const inputVisibility=dataJson.i_visibility
    const isAvailable=dataJson.available
    return { inputChannelStates,outputChannelStates,isAvailable,outputVisibility,inputVisibility }   
}
