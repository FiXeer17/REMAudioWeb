
import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"
import { clientLoader } from '../lib/clientLoader'
import SocketContextComponent from '../lib/socket/Component'
import UUIDProvider from "../lib/socket/ComponentUuid"
import UUIDLayout from "../pages/connections_socket/Redirect"

import {SignInPage as SignInDesktop } from "../pages/desktopPages/SignIn"
import {SignInPage as SignInMobile} from "@/pages/mobilePages/SignIn"

import {NewConnections as NewConnectionsDesktop } from "@/pages/desktopPages/NewConnections"
import {NewConnections as NewConnectionsMobile } from "@/pages/connections_socket/NewConnections"

import { RecentConnections as RecentConnectionsDesktop } from "@/pages/desktopPages/RecentConnections"
import { RecentConnections as RecentConnectionsMobile } from "@/pages/connections_socket/RecentConnections"

import { CreateConnections as CreateConnectionsDesktop } from "@/pages/desktopPages/CreateConnections"
import { CreateConnections as CreateConnectionsMobile  } from "@/pages/connections_socket/CreateConnections"

import { CallAdministrator as CallAdministratorDesktop } from "@/pages/desktopPages/CallAdministrator"
import { CallAdministrator as CallAdministratorMobile } from "@/pages/connections_socket/CallAdministrator"

import { HomeAudio as HomeAudioDesktop }  from "@/pages/desktopPages/HomeAudio"
import { HomeAudio as HomeAudioMobile } from "@/pages/mobilePages/HomeAudio"

import { Settings as SettingsDesktop } from "@/pages/desktopPages/Settings"
import { Settings as SettingsMobile } from "@/pages/mobilePages/Settings"

import { Volume as VolumeDesktop } from "@/pages/desktopPages/Volume"
import { Volume as VolumeMobile } from "@/pages/mobilePages/Volume"

import { useIsDesktop } from "./useDeviceType"


export const SignIn = () => {
  const isDesktop = useIsDesktop();
  return isDesktop ? <SignInDesktop /> : <SignInMobile />;
};

export const NewConnections = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <NewConnectionsDesktop /> : <NewConnectionsMobile />;
};

export const CreateConnections = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <CreateConnectionsDesktop /> : <CreateConnectionsMobile />;
};

export const RecentConnections = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <RecentConnectionsDesktop /> : <RecentConnectionsMobile />;
};

export const CallAdministrator = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <CallAdministratorDesktop /> : <CallAdministratorMobile />;
};

export const HomeAudio = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <HomeAudioDesktop /> : <HomeAudioMobile />;
};

export const Settings = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <SettingsDesktop /> : <SettingsMobile />;
};

export const Volume = () => {
    const isDesktop = useIsDesktop();
    return isDesktop ? <VolumeDesktop /> : <VolumeMobile />;
};





