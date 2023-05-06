const {
    Everything
} = require('../dist/Everything.js');

const everything = new Everything();

everything.waitDbLoaded().then(() => {
    console.log(everything.version());

    everything.setSearch("video:");

    everything.query();

    console.log(everything.getNumResults());

    for (const path of everything.pathIter()) {
        console.log(path);
    }

    everything.cleanup();
}).catch((err) => {
    console.log(err);
});
