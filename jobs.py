import pandas as pd
import plotly.graph_objects as go

# Load the CSV data
file_path = 'jobs.csv'
df = pd.read_csv(file_path)

# Convert date columns to datetime
df['Date Applied'] = pd.to_datetime(df['Date Applied'])
df['Date Responded'] = pd.to_datetime(df['Date Responded'], errors='coerce')

# Calculate the time to hear back
df['Time to Hear Back (days)'] = (df['Date Responded'] - df['Date Applied']).dt.days

# Prepare data for the Sankey diagram
sources = []
targets = []
values = []

# Status counts
status_counts = df['Status'].value_counts()
rejected_count = status_counts.get('Rejected', 0)
interview_count = status_counts.get('Interview', 0)
no_answer_count = status_counts.get('No Answer', 0)

# Total applications
total_applications = len(df)
sources.append('Total Applications')
targets.append('Rejected')
values.append(rejected_count)

sources.append('Total Applications')
targets.append('Interview')
values.append(interview_count)

sources.append('Total Applications')
targets.append('No Answer')
values.append(no_answer_count)

# Outcome counts for interviews
if interview_count > 0:
    outcome_counts = df[df['Status'] == 'Interview']['Interview'].value_counts()
    for outcome, count in outcome_counts.items():
        sources.append('Interview')
        targets.append(outcome)
        values.append(count)

# Average time to hear back
avg_time_to_hear_back = df['Time to Hear Back (days)'].mean()

# Create the labels and indices for Sankey diagram
unique_labels = list(set(sources + targets))
label_indices = {label: i for i, label in enumerate(unique_labels)}

# Create the Sankey diagram
sankey = go.Sankey(
    node=dict(
        pad=15,
        thickness=20,
        line=dict(color="black", width=0.5),
        label=unique_labels,
    ),
    link=dict(
        source=[label_indices[src] for src in sources],
        target=[label_indices[tgt] for tgt in targets],
        value=values
    )
)

# Create the figure and add annotation
fig = go.Figure(data=[sankey])
fig.update_layout(
    title_text="2024 Job Hunt",
    font_size=10,
    annotations=[
        dict(
            x=0.5, y=-0.1,
            xref='paper', yref='paper',
            text=f'Average time to hear back: {avg_time_to_hear_back:.1f} days',
            showarrow=False,
            font=dict(size=12)
        )
    ]
)

fig.show()
