import { configureStore, createReducer, Slice } from "@reduxjs/toolkit";

const ConfigurePlatyStore = () => {
    let _dependencies = {}
    let _reducers: {[key: string]: any} = {}
    
    return {
        withFeature(slice: Slice) {
            _reducers[slice.name] = slice.reducer;
            return this;
        },
        
        withDependiencies(dependencies: any) {
            _dependencies = {..._dependencies,  ...dependencies};
            return this;
        },
        
        build() {
            return configureStore({
                reducer: _reducers,
                middleware: defaultMiddleware => defaultMiddleware({thunk: {extraArgument: _dependencies}})
            })
        }
    }
}

export default ConfigurePlatyStore;