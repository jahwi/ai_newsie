# AI Newsie

## 1. Introduction
Meet AI Newsie - your personalized GPT-powered bot that delivers daily newsletters tailored to your interests.
The bot is powered by the NYT API, GPT-3-Turbo, Sendgrid API, and it is built to run on AWS Lambda. It even transcribes your newsletter to a speech recording so you can listen to it on the go.

![Newsletter Part Screenshot](/img/newsletter_body_snip.png)
![Recording Screenshot](/img/recording_snip.png)

## 2. Configuring and Deploying AI Newsie
To use the bot, one would need the following:

1. A Sendgrid API key - for sending emails
2. A NYT API key - For grabbing news stories
3. An OpenAI API Key - For processing and summarizing the news stories
4. An AWS account: 
- For access to AWS Polly (text-to-speech) and 
- AWS Lambda (deploying the bot)

### 1. Setting up Sendgrid
1. Sign up for Sendgrid and setup a [sender identity](https://docs.sendgrid.com/ui/sending-email/sender-verification) and take note of your sendgrid sender email address. The most straightforward verfication route is Single Sender Verification.
2. After setting up a sender identity, create an API key by navigating to Settings -> API Keys -> Create API Key. Take note of this API key (not to be confused with the API Key ID) as it is hidden soon after generation.

### 2. Setting up NYT API
1. Create a NYT developer account [here](https://developer.nytimes.com/).
2. Navigate to the My Apps page by clicking the dropdown at the upper left corner of the page and selecting "Apps".
3. Create an API key in the "API Keys" section by clicking "Add Key". Copy the API Key under the "Key" header.
4. Under the "APIs" section, ensure "Article Search API" is enabled.
5. Below that, hit the "Save" Button.

### 3. Setting up OpenAI API
1. Create an account [here](https://platform.openai.com/).
2. Navigate to the API Keys page by clicking your account in the top left corner of the page and selecting "View API Keys".
3. Create a new API key by clicking the "Create new secret key" button. Take note of the newly generated key.

### 4. Setting up AWS
1. If you haven't already, setup an AWS account [here](https://aws.amazon.com/).
2. Navigate to the AWS Lambda service page by searching "Lambda" in the search bar at the top of the page.
3. Click "Create Function" and select the "Author from scratch" option.
4. Create a name for the new function, e.g. "NewsBot".
5. For the "Runtime" section, select "Provide your own bootstrap on Amazon Linux 2", and under "Archictecture", select "arm64".
6. Under the "Change Default Execution Role" section, make sure "Create a new role with basic Lambda permissions" is selected.
7. Click "Create Function".
8. Add AWS Polly permissions for the bot by navigating to the "Configurations" section -> clicking "Permissions", then clicking the link under Role Name in the "Execution Role" header.
9. In the "Permissions" tab, click the "Add Permissions" button, then select "Attatch Policy".
10. Search for and select "AmazonPollyReadOnlyAccess" in the permissions search bar, then click Add Permissions.
11. Back to the Lambda Functions page, under the "Configuration" tab, select "Environment Variables".
12. Configure the environment variables as follows, by clicking the "edit" button:
![AWS Lambda Function Environment Variables](/img/env_vars.png)
13. Click the "Save" button after setting environment variables.
14. Under the "General Configuration" menu, click "Edit" and change the timeout to "2min 0sec", and save.
15. Grab a pre-built bootstrap from [here](https://github.com/jahwi/ai_newsie/releases), and downloading bootstrap.zip from the latest version.
16. Back to your AWS Lambda function, in the "Code" section, scroll down and click "Upload From" -> ".zip file".
17. Once the zip has been uploaded, test your function works by navigating to the "Test" tab and clicking the "Test" button. The function should send your AI-generated email to your desired destination, via your sendgrid sender email.
18. Check the Log Output to confirm the bot ran as expected.
19. Finally, you can schedule the bot to run daily by navigating to the "Triggers" section of the "Configuration" tab and clicking "Add Trigger".
20. Then, select EventBridge, then "Create a new rule" and configuring "Rule Name" as "Daily_Newsletter", "Rule Description" as "Run the newsbot daily", and "Schedule Expression" as "rate(1 day)", then click "Add".
![Seting a trigger](/img/trigger.png)
