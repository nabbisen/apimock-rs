name: Question
description: Ask a question about this project
labels: [question]
body:
  - type: markdown
    attributes:
      value: |
        Thank you for taking the time to ask a question! Please ensure your question hasn't already been answered in the [README](../README.md), existing issues, or discussions.

  - type: input
    id: title
    attributes:
      label: Short summary of your question
      placeholder: e.g., "How do I configure A to work with B?"
    validations:
      required: true

  - type: textarea
    id: question-details
    attributes:
      label: Your question
      description: Please provide a detailed explanation of your question.
      placeholder: |
        - What are you trying to achieve?
        - What have you tried so far?
        - What specific part is confusing or not working?
    validations:
      required: true

  - type: dropdown
    id: area
    attributes:
      label: What area does your question relate to?
      options:
        - Installation
        - Configuration
        - Usage
        - Troubleshooting
        - Development / Contribution
        - Other
