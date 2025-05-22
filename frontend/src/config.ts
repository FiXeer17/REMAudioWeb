export type RuntimeConfig = {
  APPLICATION_URL: string;
};

let config: RuntimeConfig;

export const loadConfig = async (): Promise<RuntimeConfig> => {
  if (!config) {
    const res = await fetch('./url.json');
    config = await res.json();
  }
  return config;
};

export const getConfig = (): RuntimeConfig => {
  if (!config) {
    throw new Error("Config not loaded yet");
  }
  return config!;
};
