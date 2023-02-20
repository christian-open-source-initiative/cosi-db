class ExpandedSearch {

    constructor(searchBar, render) {
        this.searchBar = searchBar;
        this.render = render;
        this.searchResults = [];

        // Grab the search bar
        this.searchBar.keyup(
            (event) => {
                let searchValue = this.searchBar.val();
                if (searchValue.length < 3) {
                    return;
                }
                this.updateRender(searchValue);
            }
        )
    }

    getExpandedRenderClass(strName) {
        if (strName == "Person") {
            return PersonExpanded;
        }

        return null;
    }

    updateRender(text) {
        // Send a search request in order to obtain things to render.
        $.get(`/search?query=${text}`, (data) => {
            // Get each row
            this.render.html("");
            this.searchResults = [];
            for (let tableKey in data) {
                let resultsPerTable = data[tableKey];

                let RenderClass = this.getExpandedRenderClass(tableKey);
                // Not yet implemented, do not render.
                if (RenderClass == null) {
                    continue;
                }

                for (let r of resultsPerTable) {
                    let d = r["data"];
                    let entryMatch = r["entry_match"];

                    // Dispatch to each unique renderer.
                    let newResult = $("<div></div>");
                    this.render.append(newResult);
                    let result = new RenderClass()
                                        .setData(d)
                                        .setDefCSSPrefix("esearch")
                                        .render(
                                            (renderHtml) => {
                                                newResult.html(renderHtml);
                                            },
                                            (err) => {
                                                console.error("Error in searching.");
                                                }
                                        );
                    this.searchResults.push(result);
                }
            }

        }).fail((d, textStatus, error) => {console.log(error);});
    }
}

$(document).ready(() => {
    let search = new ExpandedSearch($("#main-search-bar"), $("#extended-search-render"));
});
