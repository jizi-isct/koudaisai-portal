import {createContext, ReactNode, useCallback, useContext, useMemo, useState} from "react";
import {apiQueryClientType, DocumentCategoryRead, DocumentRead} from "@/lib";

interface ReadDocumentContextType {
  documents: Array<{ category: DocumentCategoryRead | null, documents: DocumentRead[] }> | undefined;
  isLoading: boolean;
  fetchError: Error | null;
  refetch: () => Promise<void>;
}

const ReadDocumentContext = createContext<ReadDocumentContextType>({
  documents: undefined,
  isLoading: false,
  fetchError: null,
  refetch: async () => {
    return
  },
});

export const useReadDocumentContext = () => useContext(ReadDocumentContext);

type ReadDocumentProviderProps = {
  children: ReactNode,
  queryClient: apiQueryClientType
}

export function ReadDocumentProvider({children, queryClient}: ReadDocumentProviderProps) {
  // state
  const [fetchError, setFetchError] = useState<Error | null>(null);

  // React Query
  const {data: documents, refetch: refetch_, isLoading} = queryClient.useQuery("get", "/documents/by-category");


  // Callbacks
  const refetch = useCallback(async () => {
    try {
      await refetch_();
    } catch (err) {
      setFetchError(err instanceof Error ? err : new Error(String(err)));
    }
  }, [refetch_]);

  const contextValue = useMemo(() => ({
    documents,
    isLoading,
    fetchError,
    refetch,
  }), [
    documents,
    isLoading,
    fetchError,
    refetch
  ]);

  return (
    <ReadDocumentContext.Provider value={contextValue}>
      {children}
    </ReadDocumentContext.Provider>
  );
}