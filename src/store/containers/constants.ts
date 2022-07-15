import { ContainersState } from "./types";

export const CONTAINERS_STORE_NAME = "containers";

export const INITIAL_STATE: ContainersState = {
    fetchingContainers: false,
    containers: []
};