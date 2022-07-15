import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { debounce } from "lodash";
import { Loading } from "components/shared";
import { ContainerItem, ContainersList } from "components/containers";
import { PlatyThunkAnyAction, PlatyThunkDispatch } from "store/types";
import { 
    CONTAINERS_STORE_NAME, 
    listContainers, 
    ContainersState, 
    ContainerSliceState 
} from "store/containers";

const debounceDispatch = debounce((
    dispatch: PlatyThunkDispatch<ContainerSliceState>, 
    action: PlatyThunkAnyAction<ContainerSliceState>
) => dispatch(action));

const ContainersScreen = () => { 
    const state = useSelector<ContainerSliceState, ContainersState>(state => state[CONTAINERS_STORE_NAME]);
    const dispatch = useDispatch();

    useEffect(() => { 
        debounceDispatch(dispatch, listContainers());
    }, []);

    return state.fetchingContainers ? <Loading/> : <ContainersList> {
        state.containers.map(container => <ContainerItem key={ container.id } container={ container }/>)
    } </ContainersList>
};

export default ContainersScreen;