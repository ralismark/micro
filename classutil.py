#!/usr/bin/env python3

import re
import click
import requests
import bs4

CLASSUTIL_URL = "http://classutil.unsw.edu.au/{}.html"
COURSE_TAG_RE = re.compile(f"[A-Z][A-Z][A-Z][A-Z][0-9][0-9][0-9][0-9]T[1-3]")

FIELD_TITLES = [
    "Comp",
    "Sect",
    "Class",
    "Type",
    "Status",
    "EnrCap",
    "PercentFull",
    "Times",
]


def load_entries(group):
    req = requests.get(CLASSUTIL_URL.format(group))
    req.raise_for_status()

    soup = bs4.BeautifulSoup(req.text, "html.parser")

    course = None

    for tr in soup.find_all("tr"):
        classes = tr.attrs.get("class", [])
        course_tag = tr.find("a", attrs={"name": COURSE_TAG_RE})
        if course_tag:
            course = course_tag["name"][0:8]
        elif any(c in classes for c in ["rowHighlight", "rowLowlight", "stub"]):
            fields = [td.get_text().strip() for td in tr.find_all("td")]
            yield (course, *fields)


@click.command()
@click.option("--group", required=True, help="e.g. MATH")
@click.option("--term", required=True, help="e.g. T3")
def main(group, term):
    print(",".join(FIELD_TITLES))
    for row in load_entries(f"{group}_{term}"):
        print(",".join(row))


if __name__ == "__main__":
    main()
