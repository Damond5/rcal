# Event File Format Specification

This document specifies the format for event files used by rcal.

## Overview

Events are stored as individual Markdown files in the `~/calendar` directory. Each event has a filename based on its title with a `.md` extension. Collisions are handled by appending a number.

## File Naming

- Filename: `{sanitized_title}.md` where title is sanitized (spaces to underscores, invalid chars removed), and a number appended for duplicates.
- Example: `Team_Meeting.md`, `Team_Meeting_1.md`

## File Structure

Each event file is a Markdown document with the following structure:

```
# Event: {title}

- **ID**: {uuid}
- **Date**: {start_date}[ to {end_date}]
- **Time**: {start_time}[ to {end_time}]
- **Description**: {description}
- **Recurrence**: {recurrence}
```

## Field Descriptions

### Title
- **Format**: Free text
- **Required**: Yes
- **Description**: The title of the event

### Date
- **Format**: `YYYY-MM-DD` or `YYYY-MM-DD to YYYY-MM-DD` for multi-day events
- **Required**: Yes (start_date required)
- **Description**: The start date of the event, optionally followed by end date for spanning events

### Time
- **Format**: `HH:MM` or `HH:MM to HH:MM` for events with duration, or `all-day` for all-day events
- **Required**: No (optional for all-day events)
- **Description**: The start time of the event, optionally followed by end time. If omitted or set to `all-day`, the event is all-day

### Description
- **Format**: Free text
- **Required**: No
- **Description**: Additional details about the event

### Recurrence
- **Format**: `none`, `daily`, `weekly`, or `monthly`
- **Required**: No (defaults to `none`)
- **Description**: How the event repeats

## Examples

### Simple Event
```
# Event: Team Meeting

- **Date**: 2023-10-01
- **Time**: 14:30
- **Description**: Weekly team sync
- **Recurrence**: none
```

### Multi-day Event
```
# Event: Conference

- **Date**: 2023-10-01 to 2023-10-03
- **Time**: 09:00 to 17:00
- **Description**: Annual developer conference
- **Recurrence**: none
```

### Recurring Event
```
# Event: Daily Standup

- **Date**: 2023-10-01
- **Time**: 09:00
- **Description**: Daily team standup
- **Recurrence**: daily
```

### All-Day Event
```
# Event: Holiday

- **Date**: 2023-10-01
- **Time**: all-day
- **Description**: National holiday
- **Recurrence**: none
```

## Parsing Rules

- All fields are parsed from the Markdown list items
- Dates must be in `YYYY-MM-DD` format
- Times must be in `HH:MM` format (24-hour) or `all-day` for all-day events
- If time is missing or invalid, the event is treated as all-day
- Empty description and recurrence default to empty string and `none` respectively
- Multi-day/time spans use ` to ` separator

## Notes

- The deprecated old format (one file per date) is no longer supported
- Events are sorted by date then time when loaded
- Recurring events generate instances automatically</content>
</xai:function_call">The event file format specification has been created in EVENT_FORMAT.md. This document provides a complete reference for the Markdown-based event storage format, including examples and parsing rules.