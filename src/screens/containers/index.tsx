import { useEffect } from "react"
import { useDispatch, useSelector } from "react-redux"
import { Loading } from "components/shared"
import { 
    CONTAINERS_STORE_NAME, 
    listContainers, 
    ContainersState, 
    ContainerSliceState 
} from "store/containers"
import {
    ContainerItem,
    ContainersList
} from "components/containers"
import { debounce } from "lodash"
import { AnyAction, ThunkAction, ThunkDispatch } from "@reduxjs/toolkit"

const debounceDispatch = debounce((dispatch: ThunkDispatch<ContainerSliceState, any, AnyAction>, action: AnyAction | ThunkAction<any, ContainerSliceState, any, AnyAction>) => dispatch(action))

const ContainersScreen = () => { 
    const state = useSelector<ContainerSliceState, ContainersState>(state => state[CONTAINERS_STORE_NAME]);
    const dispatch = useDispatch()

    useEffect(() => { 
        debounceDispatch(dispatch, listContainers())
    }, [])

    return state.fetchingContainers 
        ? <Loading/> 
        : <ContainersList> {
            state.containers.map(container => <ContainerItem key={container.id} container={container}/>)
        } </ContainersList>
}

export default ContainersScreen