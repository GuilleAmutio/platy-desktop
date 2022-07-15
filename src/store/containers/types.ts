import { PlatyReducer, PlatyReducerWithoutAction } from "store/types"
import { CONTAINERS_STORE_NAME } from "./constants"

export type Container = {
    id    : string
    name  : string
    state : "creating" | "created" | "exiting" | "exited" | "starting" | "started" | "deleting"
    // container model
}

export type ContainersState = {
    fetchingContainers: boolean,
    containers: Container[]
}

export type ContainerReducer<Payload> = PlatyReducer<ContainersState, Payload>
export type ContainerReducerWihoutAction = PlatyReducerWithoutAction<ContainersState>

export type ContainerSliceState = {
    [CONTAINERS_STORE_NAME]: ContainersState
}