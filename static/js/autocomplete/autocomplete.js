const NO_MATCHES_FOUND = ["No matches found", "", ""];

class AutoComplete {
  // if isMain is false, type of auto complete will be inline
  constructor(
    id,
    getSuggestions,
    selectSuggestion,
    isMain = true,
    placeholder = "Enter a search term",
    defaultSearch = {},
    goBackCallback = () => {}
  ) {
    this.container = document.querySelector("#" + id);
    this.getSuggestions = getSuggestions;
    this.selectSuggestion = selectSuggestion;
    this.isMain = isMain;
    this.suggestions = [];
    this.placeholder = placeholder;
    this.currentChoiceIndex = -1;
    this.inputValue = "";
    this.defaultSearch = defaultSearch;
    this.goBackCallback = goBackCallback;

    if (isMain) this.displayMain();
  }

  reDisplay(hiddenSuggestions = false) {
    const input = this.container.querySelector("input");
    const button = this.container.querySelector("#button-main");
    const xButton = this.container.querySelector("#button-x");
    const wrapper = this.container.querySelector("div");
    const suggestionsContainer = this.container.querySelector("#suggestions");

    if (xButton) {
      if(!hiddenSuggestions)
        if (input.value) {
          xButton.classList.remove("hidden");
        }
      else
        xButton.classList.add("hidden");
    }

    if (
      hiddenSuggestions ||
      !Array.isArray(this.suggestions) ||
      this.suggestions.length <= 0
    ) {
      input.classList.remove("input-has-item");
      wrapper.classList.remove("autocomplete-shadow");
      suggestionsContainer.classList.add("hidden");
      return;
    }

    suggestionsContainer.innerHTML = "";
    suggestionsContainer.classList.remove("hidden");

    input.classList.add("input-has-item");
    wrapper.classList.add("autocomplete-shadow");

    let total = this.suggestions.length;
    this.suggestions.forEach((suggestion, index) => {
      const option = this.getOptionValue(suggestion[0]);

      const packageName = document.createElement("div");
      packageName.setAttribute("class", "package-name");
      packageName.innerHTML = option;

      const packageDescription = document.createElement("div");
      packageDescription.setAttribute("class", "package-description");
      packageDescription.innerHTML = suggestion[1];

      const packageVersion = document.createElement("div")
      packageVersion.setAttribute("class", "package-version");
      packageVersion.innerHTML = suggestion[2]
      const node = document.createElement("div");
      const suggesstionContent = document.createElement("div");
      suggesstionContent.setAttribute("class", "suggestion-content");
      node.setAttribute('id', 'suggestion' + index)
      suggesstionContent.appendChild(packageName);
      suggesstionContent.appendChild(packageDescription);
      node.appendChild(suggesstionContent);
      node.appendChild(packageVersion);

      if (this.currentChoiceIndex === index)
        node.classList.add("autocomplete-item-hover");

      // add event click, mouseover and mouse leave
      if (option !== NO_MATCHES_FOUND) {
        node.addEventListener("click", () => {
          input.value = option;
          this.reDisplay(true);
          this.selectSuggestion(suggestion[0]);
        });

        node.addEventListener("mouseover", () => {
          this.getSuggestionNode(this.currentChoiceIndex)?.classList.remove(
            "autocomplete-item-hover"
          );
          node.classList.add("autocomplete-item-hover");
          this.currentChoiceIndex = index;
        });
        node.addEventListener("mouseout", () => {
          node.classList.remove("autocomplete-item-hover");
        });
      } else {
        node.classList.add("no-pointer");
      }
      suggestionsContainer.appendChild(node);
    });

  }

  getOptionValue(suggestion) {
    return typeof suggestion === "object" ? suggestion?.option : suggestion;
  }

  getSuggestionNode(id) {
    return this.container.querySelector("#suggestions #suggestion" + id);
  }

  checkKeyDown(event) {
    const key = event.key;

    const keyDownSpecial = ["ArrowUp", "ArrowDown", "Enter"];
    const emptySuggestion = this.suggestions[0] === NO_MATCHES_FOUND;
    const input = this.container.querySelector("input");
    if (key === "Enter" && !emptySuggestion) {
      // Not having any suggestion, goes to search page
      window.location.href = '/packages/search?query='+input.value;
    }
    if (!keyDownSpecial.includes(key)) return;

    const noSuggestions = this.suggestions.length;
    const pChoiceIndex = this.currentChoiceIndex;
    const pChoice = this.getSuggestionNode(pChoiceIndex);

    if (key === "ArrowUp") {
      this.currentChoiceIndex =
        pChoiceIndex >= 0 ? pChoiceIndex - 1 : noSuggestions - 1;
    } else if (key === "ArrowDown") {
      this.currentChoiceIndex =
        pChoiceIndex + 1 < noSuggestions ? pChoiceIndex + 1 : -1;
    } else if (key === "Enter") {
      if (pChoiceIndex === -1) {
        // Not choosing any suggestion, goes to search page
        window.location.href = '/packages/search?query='+input.value;
      } else {
        // Goes to highlighted package
        pChoice.click();
      }
      return;
    }

    event.preventDefault();
    if (this.currentChoiceIndex === -1) {
      input.value = this.inputValue;
    } else {
      input.value = this.getOptionValue(
        this.suggestions[this.currentChoiceIndex]
      );
    }

    const currentChoice = this.getSuggestionNode(this.currentChoiceIndex);
    pChoice?.classList.remove("autocomplete-item-hover");
    currentChoice?.classList.add("autocomplete-item-hover");
  }

  bindListeners() {
    const input = this.container.querySelector("input");
    input.addEventListener("input", async (e) => {
      if (!input.value) {
        this.suggestions = [];
      } else {
        try {
          let suggestions = await this.getSuggestions(e.target.value);
          suggestions.length > 0 ? this.suggestions = suggestions :
            input.value.length >= 3 ? this.suggestions = [NO_MATCHES_FOUND] : this.suggestions = [];
        } catch (error) {
          this.suggestions = [NO_MATCHES_FOUND];
        }
      }
      this.currentChoiceIndex = -1;
      this.inputValue = input.value;
      this.reDisplay();
    });

    input.addEventListener("keydown", (e) => this.checkKeyDown(e));

    input.addEventListener("focusin", () => this.reDisplay(false));
    input.addEventListener("focusout", () => {
      if(!$(this.container.querySelector("#suggestions")).is(":hover")) {
        window.setTimeout(() => this.reDisplay(true), 50)
      }
    });
  }

  displayMain() {
    this.container.innerHTML = `
    <div class="autocomplete-main-wrapper">
      <div class="autocomplete-main">
        <input
          class="gray"
          type="text"
          id="search-bar"
          placeholder="${this.placeholder}"
        />

        <button class="icon-default-main" id="button-main"><i class="fa fa-search"></i></button>
        <button class="icon-right-main hidden" id="button-x"><img class="icon" src="/static/resources/x-button.svg"/></i></button>
        <div id="suggestions" class="autocomplete-items autocomplete-shadow hidden"></div>
      </div>
    </div>`;

    this.bindListeners();

    const input = this.container.querySelector("input");
    const button = this.container.querySelector("#button-main");
    button.addEventListener("click", () => {
      window.location.href = '/packages/search?query='+input.value;
    });
    const xButton = this.container.querySelector("#button-x");
    xButton.addEventListener("click", () => {
      input.value = "";
      this.suggestions = [];
      xButton.classList.add("hidden");
    });
  }
}
