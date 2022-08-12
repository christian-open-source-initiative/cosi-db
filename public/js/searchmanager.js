class SearchManager {
    constructor(searchBar, searchSuggestion, searchButton, searchDarkener) {
        this.searchBar = searchBar;
        this.searchSuggestion = searchSuggestion;
        this.searchButton = searchButton;
        this.searchDarkener = searchDarkener;

        // Initial states
        this.searchSuggestion.hide();
        this.searchDarkener.hide();
        // We don't want to spam the server
        this.currentQuery = "";

        // Requires binding due to function reference.
        this.searchBar.keyup(
            (event) => {
                if (event.key == "Escape") {
                    this.searchDarkener.hide();
                    this.searchSuggestion.hide();
                    // Unfocus so that we can refocus if start typing.
                    this.searchDarkener.blur();
                    return;
                }
                this.determineHide();
                if (this.currentQuery.length != this.searchBar.val().length) {
                    this.currentQuery = this.searchBar.val();
                    this.dispatchSearch();
                }
            }
        );

        // We only want to hid if user focuses and already typed.
        this.searchBar.focus(this.determineHide.bind(this));
        this.searchBar.blur(() => { 
            // Provide some delay for click on suggestions. A bit hacky. TODO: Add selection tracking aware.
            setTimeout(() => {this.searchDarkener.hide(); this.searchSuggestion.hide()}, 100);
        });
    }

    dispatchSearch() {
        if (this.currentQuery.length < 3) {
            return;
        }

        // Dispatches the search result to all available tables.
        console.log(this.currentQuery)
        $.get(`/search?query=${this.currentQuery}`, (data) => {
            this.updateSearchSuggestions(data);
        }).fail((d, textStatus, error) => {console.log(error);});
    }

    updateSearchSuggestions(data) {
        this.searchSuggestion.empty();
        let finalRenderPerTable = {};
        for (let tableKey in data) {
            let resultsPerTable = data[tableKey];
            finalRenderPerTable[tableKey] = [];
            for (let r of resultsPerTable) {
                let d = r["data"];
                let entryMatch = r["entry_match"];

                // Find the highlight and insert highlight tag.
                let matchData = d[entryMatch];
                let idx = matchData.toLowerCase().indexOf(this.currentQuery.toLowerCase());
                let eidx = idx + this.currentQuery.length;
                let textWrap = `<mark class="search-highlight"> ${matchData.substring(idx, eidx)}</mark>`;
                let matchDataHighlight = matchData.substring(0, idx - 1) + textWrap + matchData.substring(eidx);

                // Render generation.
                let searchResult = `<div class="search-suggestion-result" data="${matchData}">${matchDataHighlight}</div>`
                let searchTable = `<div class="search-suggestion-table" entry="${entryMatch}" table="${tableKey}">${tableKey}::${entryMatch}</div>`
                let render = `<div class="search-suggestion-entry">${searchResult}${searchTable}</div>`
                finalRenderPerTable[tableKey].push(render);
            }
        }

        // Bound each table to at most n results for now.
        for (let tableKey in finalRenderPerTable) {
            const n = Math.min(3, finalRenderPerTable[tableKey].length);
            for (let i = 0; i < n; ++i) {
                this.searchSuggestion.append(finalRenderPerTable[tableKey][i]);
            }
        }
        this.searchSuggestion.show();
    }

    determineHide() {
        if (this.searchBar.val() == "") {
            this.searchDarkener.hide();
            this.searchSuggestion.hide();
            return true;
        }

        this.searchSuggestion.show();
        this.searchDarkener.show();
    }
}
