import * as vscode from 'vscode';

interface PredictResponse {
    suggestions: string[];
}

export function activate(context: vscode.ExtensionContext) {
    console.log("Autocomplete extension activated");

    const provider = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file' },
        {
            async provideCompletionItems(document, position) {

                // Get text before cursor
                const line = document.lineAt(position).text.substring(0, position.character);

                if (!line || line.trim().split(/\s+/).length < 2) {
                    return undefined;
                }

                try {
                    const response = await fetch("http://127.0.0.1:3000/predict", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify({ context: line })
                    });

                    const data = await response.json() as PredictResponse;

                    if (!data.suggestions || data.suggestions.length === 0) {
                        return undefined;
                    }

                    return data.suggestions.map(word => {
                        return new vscode.CompletionItem(
                            word,
                            vscode.CompletionItemKind.Text
                        );
                    });

                } catch (error) {
                    console.error("Prediction error:", error);
                    return undefined;
                }
            }
        },
        ' ' // trigger on space
    );

    context.subscriptions.push(provider);
}
