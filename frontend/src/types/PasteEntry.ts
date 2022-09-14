export class PasteEntry {
  hash: string;
  title: string;
  body: string;
  creation_date: string;
  click_count: number;

  constructor(
    hash: string,
    title: string,
    body: string,
    creation_date: string,
    click_count: number
  ) {
    this.hash = hash;
    this.title = title;
    this.body = body;
    this.creation_date = creation_date;
    this.click_count = click_count;
  }
}
