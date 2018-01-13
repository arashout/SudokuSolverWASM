document.addEventListener('DOMContentLoaded', () => { 
    Rust.solver.then( (solver) => {
        console.log(solver);
        console.log('rust code!');
    })
    .finally( () => {
        console.log('dsfasdf');
    });
}, false);