export function GetData(data:string){
    const dataJson= JSON.parse(data)
    const inputChannelStates=dataJson.i_mute
    const outputChannelStates=dataJson.o_mute
    const inputVolumesStates=dataJson.i_volumes
    const outputVolumesStates=dataJson.o_volumes
    const outputVisibility=dataJson.o_visibility
    const inputVisibility=dataJson.i_visibility
    const isAvailable=dataJson.available
    const currentPresets=dataJson.current_preset
    const labelPresets=dataJson.preset_labels
    const labelChannelsOutput=dataJson.o_labels
    const labelChannelsInput=dataJson.i_labels
    const mix_map=dataJson.mix_map

    return { inputChannelStates,outputChannelStates,inputVolumesStates,outputVolumesStates,isAvailable,outputVisibility,inputVisibility,currentPresets,labelPresets,labelChannelsInput,labelChannelsOutput,mix_map }   
}
