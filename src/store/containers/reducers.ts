import { Container, ContainerReducer, ContainerReducerWihoutAction } from "./types"

// Example of normarl reducer, wihtout thunk
export const _toggleView: ContainerReducer<{view: string}> = (state, action) => {
    // do something
    // no functionality just for testing
}

// reducers linked to thunk events

export const _beginListContainers: ContainerReducerWihoutAction = (state) => {
    // do something
    state.fetchingContainers = true
} 

export const _containersListed: ContainerReducer<Container[]> = (state, action) => {
    // do something
    state.fetchingContainers = false
    state.containers = action.payload
}

export const _containersListingRejected: ContainerReducerWihoutAction = (state) => {
    // do something
    // state.fetchingContainers = false
}