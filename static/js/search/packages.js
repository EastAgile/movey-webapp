class Packages {
    constructor() {
        this.sortElement = $("select[name='packages-sort']");
        this.init();
    }

    init() {
        this.sortElement.select2({
            minimumResultsForSearch: -1,
            dropdownCssClass: 'packages-sort-dropdown'
        });
        this.sortElement.val(this.sortElement.data('sort')).trigger('change');

        this.sortElement.on('change', function(e) {
            const $elem = $(e.currentTarget);
            window.location = $elem.data('url') + '&sort_type=' + $elem.val();
        });
    }
}


function sec_to_ymdhms(sec) {
    let years_diff = Math.floor(sec / 31536000);
    let months_diff = Math.floor((sec % 31536000) / 2628000);
    let days_diff = Math.floor(((sec % 31536000) % 2628000) / 86400);
    let hours_diff = Math.floor((((sec % 31536000) % 2628000) % 86400) / 3600);
    let mins_diff = Math.floor(((((sec % 31536000) % 2628000) % 86400) % 3600) / 60);
    let secs_diff = ((((sec % 31536000) % 2628000) % 86400) % 3600) % 60;
    return {
        year: years_diff,
        month: months_diff,
        day: days_diff,
        hour: hours_diff,
        minute: mins_diff,
        second: secs_diff
    }
}

function displayUpdateTime(el) {
    let $timestamp = $(el);
    let last_update = Date.parse($timestamp.data('last-update'));
    let diff = Math.floor((Date.now() - last_update) / 1000);
    diff = sec_to_ymdhms(diff);
    diff_text = ""
    

    if (diff.year){
        diff_text = diff.year.toString() + " years";
    }
    else if (diff.month) {
        diff_text = diff.month.toString() + " months";
    }
    else if (diff.day) {
        diff_text = diff.day.toString() + " days";
    }
    else if (diff.hour) {
        diff_text = diff.hour.toString() + " hours";
    }
    else if (diff.minute) {
        diff_text = diff.minute.toString() + " minutes";
    }
    else {
        diff_text = diff.second.toString() + " seconds";
    }
    $timestamp.children('span').text("Updated " + diff_text + " ago");
}

