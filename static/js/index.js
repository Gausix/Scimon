window.onload = e => {

    Licenses.init();
    Citations.init();
    DocsSources.init();

    ScrollTo.checkScroll();
    window.addEventListener('scroll', ScrollTo.checkScroll);

    let pageTitle = document.title.split(':');
    document.getElementById(Elements.headerLabel).textContent = `${ pageTitle[1] }: ${ pageTitle[2] }`;

    document.getElementById(ScrollTo.topBtn).addEventListener('click', ScrollTo.top);
    document.getElementById(Citations.toggleBtn).addEventListener('click', Citations.scrollTo);
    document.getElementById(DocsSources.toggleBtn).addEventListener('click', DocsSources.scrollTo);
    document.getElementById(Licenses.toggleBtn).addEventListener('click', Licenses.toggleLicensesBox);
    
};
