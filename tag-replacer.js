window.onload = function () {
    replaceTagWithSpanClass("mask", "mask");
}

function replaceTagWithSpanClass(tag, clazz) {
    let elems = document.getElementsByTagName(tag);
    do {
        for (elem of elems) {
            elem.outerHTML = elem.outerHTML.replace(`<${tag}>`, `<span class="${clazz}">`);
            elem.innerHTML = elem.innerHTML.replace(`</${tag}>`, '</span>');
        }
        elems = document.getElementsByTagName(tag);
    } while (elems.length > 0)
}
